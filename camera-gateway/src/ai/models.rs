use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoundingBox {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub width: f64,
    pub height: f64,
    pub center_x: f64,
    pub center_y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Detection {
    pub class_id: i64,
    pub class_name: String,
    pub confidence: f64,
    pub bbox: BoundingBox,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageSize {
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InferenceMetadata {
    pub model_name: String,
    pub image_size: ImageSize,
    pub source: String,
    pub confidence_threshold: f64,
    pub iou_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DetectionResponse {
    pub detections: Vec<Detection>,
    pub detection_count: i64,
    pub image_size: ImageSize,
    pub processing_time_ms: f64,
    pub inference_time_ms: f64,
    pub metadata: InferenceMetadata,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_response() -> DetectionResponse {
        DetectionResponse {
            detections: vec![Detection {
                class_id: 0,
                class_name: "person".to_string(),
                confidence: 0.95,
                bbox: BoundingBox {
                    x1: 10.0,
                    y1: 20.0,
                    x2: 100.0,
                    y2: 200.0,
                    width: 90.0,
                    height: 180.0,
                    center_x: 55.0,
                    center_y: 110.0,
                },
            }],
            detection_count: 1,
            image_size: ImageSize {
                width: 1920,
                height: 1080,
            },
            processing_time_ms: 42.5,
            inference_time_ms: 30.1,
            metadata: InferenceMetadata {
                model_name: "yolov8n".to_string(),
                image_size: ImageSize {
                    width: 1920,
                    height: 1080,
                },
                source: "camera-1".to_string(),
                confidence_threshold: 0.5,
                iou_threshold: 0.45,
            },
        }
    }

    #[test]
    fn test_serialization_roundtrip() {
        let resp = sample_response();
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: DetectionResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, parsed);
    }

    #[test]
    fn test_empty_detections() {
        let resp = DetectionResponse {
            detections: vec![],
            detection_count: 0,
            ..sample_response()
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"detections\":[]"));
    }

    #[test]
    fn test_deserialize_from_json() {
        let json = r#"{
            "detections": [],
            "detection_count": 0,
            "image_size": {"width": 640, "height": 480},
            "processing_time_ms": 1.0,
            "inference_time_ms": 0.5,
            "metadata": {
                "model_name": "test",
                "image_size": {"width": 640, "height": 480},
                "source": "test",
                "confidence_threshold": 0.5,
                "iou_threshold": 0.45
            }
        }"#;
        let resp: DetectionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.detection_count, 0);
        assert_eq!(resp.image_size.width, 640);
    }

    #[test]
    fn test_bounding_box_values() {
        let bb = BoundingBox {
            x1: 0.0,
            y1: 0.0,
            x2: 100.0,
            y2: 50.0,
            width: 100.0,
            height: 50.0,
            center_x: 50.0,
            center_y: 25.0,
        };
        assert_eq!(bb.center_x, 50.0);
        assert_eq!(bb.center_y, 25.0);
    }

    #[test]
    fn test_clone() {
        let resp = sample_response();
        let cloned = resp.clone();
        assert_eq!(resp, cloned);
    }

    #[test]
    fn test_debug() {
        let resp = sample_response();
        let debug = format!("{resp:?}");
        assert!(debug.contains("DetectionResponse"));
        assert!(debug.contains("person"));
    }
}
