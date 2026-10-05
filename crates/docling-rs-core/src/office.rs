//! Bounded, non-executing Office package reader. Does not fetch external resources.
use crate::{ConversionError, DocumentSource, InputDocument};
use std::{
    collections::BTreeMap,
    io::{Cursor, Read},
};
pub fn error(message: impl Into<String>) -> ConversionError {
    ConversionError::ParseError(message.into())
}
/// XML parts only; media remains unexpanded. All paths stay inside the package.
pub fn read_parts(input: &InputDocument) -> Result<BTreeMap<String, String>, ConversionError> {
    const LIMIT: u64 = 256 * 1024 * 1024;
    let bytes = match input.source() {
        DocumentSource::FilePath(p) => {
            let file = std::fs::File::open(p)?;
            let mut b = Vec::new();
            file.take(LIMIT + 1).read_to_end(&mut b)?;
            b
        }
        DocumentSource::Bytes { data, .. } => {
            if data.len() as u64 > LIMIT {
                return Err(error("Office input exceeds 256 MiB limit"));
            }
            data.clone()
        }
    };
    if bytes.len() as u64 > LIMIT {
        return Err(error("Office input exceeds 256 MiB limit"));
    }
    let mut zip =
        zip::ZipArchive::new(Cursor::new(bytes)).map_err(|e| error(format!("Office ZIP: {e}")))?;
    if zip.len() > 20000 {
        return Err(error("Office package has too many entries"));
    }
    let mut parts = BTreeMap::new();
    let mut total = 0u64;
    for i in 0..zip.len() {
        let mut entry = zip
            .by_index(i)
            .map_err(|e| error(format!("Office ZIP entry: {e}")))?;
        let name = entry.name().to_owned();
        if !(name.ends_with(".xml") || name.ends_with(".rels")) {
            continue;
        }
        if entry.size() > 16 * 1024 * 1024 {
            return Err(error("Office XML part exceeds 16 MiB limit"));
        }
        total = total
            .checked_add(entry.size())
            .ok_or_else(|| error("Office size overflow"))?;
        if total > 64 * 1024 * 1024 {
            return Err(error("Office XML exceeds 64 MiB limit"));
        }
        let mut data = Vec::new();
        (&mut entry)
            .take(16 * 1024 * 1024 + 1)
            .read_to_end(&mut data)
            .map_err(|e| error(format!("Office ZIP integrity: {e}")))?;
        if data.len() > 16 * 1024 * 1024 {
            return Err(error("Office XML expansion limit"));
        }
        let xml =
            String::from_utf8(data).map_err(|e| error(format!("Office XML encoding: {e}")))?;
        // Reject malformed/DTD input and excessive nesting before backend traversal.
        let parsed = roxmltree::Document::parse(&xml)
            .map_err(|e| error(format!("Office XML {name}: {e}")))?;
        if parsed
            .descendants()
            .any(|n| n.ancestors().take(130).count() > 128)
        {
            return Err(error("Office XML nesting exceeds 128 levels"));
        }
        if parts.insert(name, xml).is_some() {
            return Err(error("Duplicate Office XML part"));
        }
    }
    Ok(parts)
}
pub fn name(input: &InputDocument) -> String {
    match input.source() {
        DocumentSource::FilePath(p) => p
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        DocumentSource::Bytes { name, .. } => name.clone(),
    }
}
pub fn parse(xml: &str) -> Result<roxmltree::Document<'_>, ConversionError> {
    roxmltree::Document::parse(xml).map_err(|e| error(format!("Office XML: {e}")))
}
pub fn part<'a>(
    parts: &'a BTreeMap<String, String>,
    path: &str,
) -> Result<&'a str, ConversionError> {
    parts
        .get(path)
        .map(String::as_str)
        .ok_or_else(|| error(format!("Missing Office part: {path}")))
}
/// Resolve an internal OPC relationship. External targets are deliberately not resolved.
pub fn resolve(base: &str, target: &str) -> Result<String, ConversionError> {
    if target.contains('\\') || target.contains(':') {
        return Err(error("Invalid Office relationship target"));
    }
    let path = if target.starts_with('/') {
        target.trim_start_matches('/').to_owned()
    } else {
        format!(
            "{}/{}",
            base.rsplit_once('/').map(|(p, _)| p).unwrap_or(""),
            target
        )
    };
    let mut segments = vec![];
    for s in path.split('/') {
        match s {
            "" | "." => {}
            ".." => {
                if segments.pop().is_none() {
                    return Err(error("Office relationship escapes package"));
                }
            }
            _ => segments.push(s),
        }
    }
    Ok(segments.join("/"))
}
pub fn relationships(
    parts: &BTreeMap<String, String>,
    base: &str,
) -> Result<BTreeMap<String, (String, String)>, ConversionError> {
    let (dir, file) = base.rsplit_once('/').unwrap_or(("", base));
    let path = format!("{dir}/_rels/{file}.rels");
    let mut result = BTreeMap::new();
    if let Some(xml) = parts.get(&path) {
        let doc = parse(xml)?;
        for n in doc.descendants().filter(|n| n.has_tag_name("Relationship")) {
            if n.attribute("TargetMode") == Some("External") {
                continue;
            }
            if let (Some(id), Some(target)) = (n.attribute("Id"), n.attribute("Target")) {
                result.insert(
                    id.to_owned(),
                    (
                        resolve(base, target)?,
                        n.attribute("Type").unwrap_or("").to_owned(),
                    ),
                );
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    fn package(xml: &str) -> InputDocument {
        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        zip.start_file(
            "word/document.xml",
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored),
        )
        .unwrap();
        zip.write_all(xml.as_bytes()).unwrap();
        InputDocument::from_bytes(
            zip.finish().unwrap().into_inner(),
            "test.docx",
            crate::InputFormat::Docx,
        )
    }
    #[test]
    fn rejects_malformed_xml_and_dtd() {
        assert!(read_parts(&package("<root>")).is_err());
        assert!(read_parts(&package(
            "<!DOCTYPE root [<!ENTITY x 'secret'>]><root>&x;</root>"
        ))
        .is_err());
    }
    #[test]
    fn rejects_deep_and_oversized_parts() {
        let deep = format!("{}text{}", "<a>".repeat(130), "</a>".repeat(130));
        assert!(read_parts(&package(&deep)).is_err());
        assert!(read_parts(&package(&"x".repeat(16 * 1024 * 1024 + 1))).is_err());
    }
    #[test]
    fn relationship_resolution_stays_inside_package() {
        assert_eq!(
            resolve("ppt/slides/slide1.xml", "../notesSlides/notesSlide1.xml").unwrap(),
            "ppt/notesSlides/notesSlide1.xml"
        );
        assert!(resolve("word/document.xml", "../../outside.xml").is_err());
        assert!(resolve("word/document.xml", "https://example.invalid/document.xml").is_err());
    }
}
