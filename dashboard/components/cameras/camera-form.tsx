"use client";

import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { createCameraSchema, updateCameraSchema, type CreateCameraFormData, type UpdateCameraFormData } from "@/lib/validators";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { useCreateCamera, useUpdateCamera, useCamera } from "@/hooks/use-cameras";

interface CameraFormProps {
  cameraId?: string;
  onSuccess?: () => void;
  onCancel?: () => void;
}

export function CameraForm({ cameraId, onSuccess, onCancel }: CameraFormProps) {
  const isEditing = !!cameraId;
  const { data: existingCamera } = useCamera(cameraId || "");
  const createMutation = useCreateCamera();
  const updateMutation = useUpdateCamera();

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<CreateCameraFormData>({
    resolver: zodResolver(isEditing ? updateCameraSchema : createCameraSchema),
    values: isEditing && existingCamera ? {
      name: existingCamera.name,
      location: existingCamera.location || "",
      rtsp_url: existingCamera.rtsp_url,
      fps: existingCamera.fps || undefined,
      resolution: existingCamera.resolution || "",
    } : undefined,
  });

  const onSubmit = (data: CreateCameraFormData) => {
    if (isEditing && cameraId) {
      updateMutation.mutate(
        { id: cameraId, data: data as UpdateCameraFormData },
        { onSuccess }
      );
    } else {
      createMutation.mutate(data, { onSuccess });
    }
  };

  const isPending = createMutation.isPending || updateMutation.isPending;

  return (
    <Card>
      <CardHeader>
        <CardTitle>{isEditing ? "Edit Camera" : "Create Camera"}</CardTitle>
        <CardDescription>
          {isEditing ? "Update camera configuration" : "Add a new camera to the system"}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="name">Name</Label>
            <Input id="name" {...register("name")} placeholder="Camera name" />
            {errors.name && <p className="text-xs text-destructive">{errors.name.message}</p>}
          </div>
          <div className="space-y-2">
            <Label htmlFor="rtsp_url">RTSP URL</Label>
            <Input id="rtsp_url" {...register("rtsp_url")} placeholder="rtsp://192.168.1.100:554/stream" />
            {errors.rtsp_url && <p className="text-xs text-destructive">{errors.rtsp_url.message}</p>}
          </div>
          <div className="space-y-2">
            <Label htmlFor="location">Location</Label>
            <Input id="location" {...register("location")} placeholder="Building A, Floor 2" />
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="fps">FPS</Label>
              <Input id="fps" type="number" {...register("fps", { valueAsNumber: true })} placeholder="15" />
            </div>
            <div className="space-y-2">
              <Label htmlFor="resolution">Resolution</Label>
              <Input id="resolution" {...register("resolution")} placeholder="1920x1080" />
            </div>
          </div>
          <div className="flex justify-end gap-2">
            {onCancel && <Button type="button" variant="outline" onClick={onCancel}>Cancel</Button>}
            <Button type="submit" disabled={isPending}>
              {isPending ? "Saving..." : isEditing ? "Update" : "Create"}
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  );
}
