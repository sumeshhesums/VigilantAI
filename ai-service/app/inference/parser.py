"""Parser for converting Ultralytics YOLO results to DetectionResponse."""

from __future__ import annotations

import numpy as np

from app.inference.results import (
    BoundingBox,
    Detection,
    DetectionResponse,
    ImageSize,
    InferenceMetadata,
)
from app.logging import get_logger

logger = get_logger(__name__)


class ResultParser:
    """Parses raw Ultralytics YOLO results into DetectionResponse.

    Accepts the results from `model.predict()` and extracts
    bounding boxes, class information, and confidence scores.
    """

    def parse(
        self,
        results: list,
        image_width: int,
        image_height: int,
        model_name: str,
        confidence_threshold: float,
        iou_threshold: float,
        source: str = "<unknown>",
    ) -> DetectionResponse:
        """Parse Ultralytics results into DetectionResponse.

        Args:
            results: List of Ultralytics Results objects.
            image_width: Original image width.
            image_height: Original image height.
            model_name: Name of the model used.
            confidence_threshold: Applied confidence threshold.
            iou_threshold: Applied IoU threshold.
            source: Image source identifier.

        Returns:
            DetectionResponse with parsed detections.
        """
        detections: list[Detection] = []

        if not results:
            return DetectionResponse(
                detections=[],
                detection_count=0,
                image_size=ImageSize(width=image_width, height=image_height),
                processing_time_ms=0.0,
                inference_time_ms=0.0,
                metadata=InferenceMetadata(
                    model_name=model_name,
                    image_size=ImageSize(width=image_width, height=image_height),
                    source=source,
                    confidence_threshold=confidence_threshold,
                    iou_threshold=iou_threshold,
                ),
            )

        result = results[0]

        boxes = result.boxes
        if boxes is None or len(boxes) == 0:
            return DetectionResponse(
                detections=[],
                detection_count=0,
                image_size=ImageSize(width=image_width, height=image_height),
                processing_time_ms=0.0,
                inference_time_ms=0.0,
                metadata=InferenceMetadata(
                    model_name=model_name,
                    image_size=ImageSize(width=image_width, height=image_height),
                    source=source,
                    confidence_threshold=confidence_threshold,
                    iou_threshold=iou_threshold,
                ),
            )

        names = result.names if hasattr(result, "names") else {}
        xyxy = boxes.xyxy.cpu().numpy()
        confs = boxes.conf.cpu().numpy()
        clss = boxes.cls.cpu().numpy().astype(int)

        for i in range(len(xyxy)):
            conf = float(confs[i])
            if conf < confidence_threshold:
                continue

            x1, y1, x2, y2 = xyxy[i].tolist()
            cls_id = int(clss[i])
            class_name = names.get(cls_id, f"class_{cls_id}")

            detection = Detection(
                class_id=cls_id,
                class_name=class_name,
                confidence=round(conf, 6),
                bbox=BoundingBox.from_xyxy(
                    x1=max(0.0, x1),
                    y1=max(0.0, y1),
                    x2=min(float(image_width), x2),
                    y2=min(float(image_height), y2),
                ),
            )
            detections.append(detection)

        logger.debug(
            "Parsed %d detections (threshold=%.2f)",
            len(detections),
            confidence_threshold,
        )

        return DetectionResponse(
            detections=detections,
            detection_count=len(detections),
            image_size=ImageSize(width=image_width, height=image_height),
            processing_time_ms=0.0,
            inference_time_ms=0.0,
            metadata=InferenceMetadata(
                model_name=model_name,
                image_size=ImageSize(width=image_width, height=image_height),
                source=source,
                confidence_threshold=confidence_threshold,
                iou_threshold=iou_threshold,
            ),
        )

    def filter_by_confidence(
        self,
        detections: list[Detection],
        threshold: float,
    ) -> list[Detection]:
        """Filter detections by confidence threshold.

        Args:
            detections: List of Detection objects.
            threshold: Minimum confidence score.

        Returns:
            Filtered list of detections.
        """
        return [d for d in detections if d.confidence >= threshold]

    def detections_to_numpy(self, detections: list[Detection]) -> np.ndarray:
        """Convert detections to NumPy array for downstream use.

        Returns array of shape (N, 6) with columns:
        [x1, y1, x2, y2, confidence, class_id].

        Args:
            detections: List of Detection objects.

        Returns:
            NumPy array of shape (N, 6) or empty (0, 6).
        """
        if not detections:
            return np.zeros((0, 6), dtype=np.float32)

        rows = []
        for d in detections:
            rows.append(
                [
                    d.bbox.x1,
                    d.bbox.y1,
                    d.bbox.x2,
                    d.bbox.y2,
                    d.confidence,
                    float(d.class_id),
                ]
            )
        return np.array(rows, dtype=np.float32)
