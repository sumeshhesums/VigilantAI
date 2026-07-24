import { loginSchema, createUserSchema, createCameraSchema } from "@/lib/validators";

describe("loginSchema", () => {
  it("validates correct login data", () => {
    const result = loginSchema.safeParse({
      email: "user@example.com",
      password: "password123",
    });
    expect(result.success).toBe(true);
  });

  it("rejects invalid email", () => {
    const result = loginSchema.safeParse({
      email: "not-an-email",
      password: "password123",
    });
    expect(result.success).toBe(false);
  });

  it("rejects empty password", () => {
    const result = loginSchema.safeParse({
      email: "user@example.com",
      password: "",
    });
    expect(result.success).toBe(false);
  });
});

describe("createUserSchema", () => {
  it("validates correct user data", () => {
    const result = createUserSchema.safeParse({
      email: "user@example.com",
      password: "StrongP@ss1w0rd",
      first_name: "John",
      last_name: "Doe",
    });
    expect(result.success).toBe(true);
  });

  it("rejects short password", () => {
    const result = createUserSchema.safeParse({
      email: "user@example.com",
      password: "short",
      first_name: "John",
      last_name: "Doe",
    });
    expect(result.success).toBe(false);
  });

  it("rejects password without uppercase", () => {
    const result = createUserSchema.safeParse({
      email: "user@example.com",
      password: "nouppercase1@!",
      first_name: "John",
      last_name: "Doe",
    });
    expect(result.success).toBe(false);
  });
});

describe("createCameraSchema", () => {
  it("validates correct camera data", () => {
    const result = createCameraSchema.safeParse({
      name: "Lobby Camera",
      rtsp_url: "rtsp://192.168.1.100:554/stream",
    });
    expect(result.success).toBe(true);
  });

  it("rejects invalid RTSP URL", () => {
    const result = createCameraSchema.safeParse({
      name: "Lobby Camera",
      rtsp_url: "not-a-url",
    });
    expect(result.success).toBe(false);
  });

  it("rejects empty name", () => {
    const result = createCameraSchema.safeParse({
      name: "",
      rtsp_url: "rtsp://192.168.1.100:554/stream",
    });
    expect(result.success).toBe(false);
  });
});
