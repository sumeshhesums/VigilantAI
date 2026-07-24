import { SEVERITY_COLORS, STATUS_COLORS, INCIDENT_STATUS_COLORS, PAGE_SIZES, DEFAULT_PAGE_SIZE, NAV_ITEMS } from "@/lib/constants";

describe("constants", () => {
  it("has severity colors for all levels", () => {
    expect(SEVERITY_COLORS).toHaveProperty("critical");
    expect(SEVERITY_COLORS).toHaveProperty("high");
    expect(SEVERITY_COLORS).toHaveProperty("medium");
    expect(SEVERITY_COLORS).toHaveProperty("low");
  });

  it("has status colors", () => {
    expect(STATUS_COLORS).toHaveProperty("online");
    expect(STATUS_COLORS).toHaveProperty("offline");
    expect(STATUS_COLORS).toHaveProperty("maintenance");
  });

  it("has incident status colors", () => {
    expect(INCIDENT_STATUS_COLORS).toHaveProperty("open");
    expect(INCIDENT_STATUS_COLORS).toHaveProperty("acknowledged");
    expect(INCIDENT_STATUS_COLORS).toHaveProperty("resolved");
    expect(INCIDENT_STATUS_COLORS).toHaveProperty("false_positive");
  });

  it("has page sizes", () => {
    expect(PAGE_SIZES).toEqual([10, 20, 50, 100]);
    expect(DEFAULT_PAGE_SIZE).toBe(20);
  });

  it("has navigation items", () => {
    expect(NAV_ITEMS.length).toBeGreaterThan(0);
    expect(NAV_ITEMS[0]).toHaveProperty("title");
    expect(NAV_ITEMS[0]).toHaveProperty("href");
    expect(NAV_ITEMS[0]).toHaveProperty("icon");
  });
});
