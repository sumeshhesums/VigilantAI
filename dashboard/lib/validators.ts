import { z } from "zod";

export const loginSchema = z.object({
  email: z.string().email("Invalid email address"),
  password: z.string().min(1, "Password is required"),
});

export const createUserSchema = z.object({
  email: z.string().email("Invalid email address"),
  password: z
    .string()
    .min(12, "Password must be at least 12 characters")
    .regex(/[A-Z]/, "Must contain at least one uppercase letter")
    .regex(/[a-z]/, "Must contain at least one lowercase letter")
    .regex(/[0-9]/, "Must contain at least one number")
    .regex(/[^A-Za-z0-9]/, "Must contain at least one special character"),
  first_name: z.string().min(1, "First name is required"),
  last_name: z.string().min(1, "Last name is required"),
});

export const updateUserSchema = z.object({
  email: z.string().email("Invalid email address").optional(),
  first_name: z.string().min(1, "First name is required").optional(),
  last_name: z.string().min(1, "Last name is required").optional(),
});

export const createCameraSchema = z.object({
  name: z.string().min(1, "Camera name is required").max(150),
  location: z.string().optional(),
  rtsp_url: z.string().url("Must be a valid RTSP URL"),
  fps: z.number().int().min(1).max(60).optional(),
  resolution: z.string().optional(),
});

export const updateCameraSchema = z.object({
  name: z.string().min(1).max(150).optional(),
  location: z.string().nullable().optional(),
  rtsp_url: z.string().url("Must be a valid RTSP URL").optional(),
  fps: z.number().int().min(1).max(60).nullable().optional(),
  resolution: z.string().nullable().optional(),
  enabled: z.boolean().optional(),
});

export const createIncidentSchema = z.object({
  camera_id: z.string().uuid("Invalid camera ID"),
  severity: z.enum(["low", "medium", "high", "critical"]),
  event_type: z.string().min(1, "Event type is required"),
  confidence: z.number().min(0).max(1),
  timestamp: z.string().optional(),
});

export const sendNotificationSchema = z.object({
  incident_id: z.string().uuid("Invalid incident ID"),
  channel: z.enum(["email", "webhook", "dashboard"]),
  recipient: z.string().min(1, "Recipient is required"),
});

export const assignRoleSchema = z.object({
  role: z.string().min(1, "Role name is required"),
});

export type LoginFormData = z.infer<typeof loginSchema>;
export type CreateUserFormData = z.infer<typeof createUserSchema>;
export type UpdateUserFormData = z.infer<typeof updateUserSchema>;
export type CreateCameraFormData = z.infer<typeof createCameraSchema>;
export type UpdateCameraFormData = z.infer<typeof updateCameraSchema>;
export type CreateIncidentFormData = z.infer<typeof createIncidentSchema>;
export type SendNotificationFormData = z.infer<typeof sendNotificationSchema>;
export type AssignRoleFormData = z.infer<typeof assignRoleSchema>;
