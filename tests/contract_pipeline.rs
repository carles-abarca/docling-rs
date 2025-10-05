//! Contract tests for Pipeline trait

use docling_rs::datamodel::{ConversionResult, InputDocument};
use docling_rs::error::ConversionError;
use docling_rs::pipeline::Pipeline;
use docling_rs::InputFormat;
use std::path::PathBuf;

struct MockPipeline;

impl Pipeline for MockPipeline {
    fn execute(&self, input: &InputDocument) -> Result<ConversionResult, ConversionError> {
        let doc = docling_rs::datamodel::DoclingDocument::new("test");
        let result = ConversionResult::new(doc, docling_rs::datamodel::ConversionStatus::Success);
        Ok(result)
    }
}

#[test]
fn test_pipeline_execute() {
    let pipeline = MockPipeline;
    let input = InputDocument::from_path(PathBuf::from("test.md"), InputFormat::Markdown);

    let result = pipeline.execute(&input);
    assert!(result.is_ok());

    let conv_result = result.unwrap();
    assert_eq!(
        conv_result.status(),
        docling_rs::datamodel::ConversionStatus::Success
    );
}
