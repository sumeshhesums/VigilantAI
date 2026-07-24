import { NextResponse } from "next/server";

const startTime = Date.now();

export async function GET() {
  const uptimeSeconds = Math.floor((Date.now() - startTime) / 1000);

  const metrics = [
    "# HELP vigilantai_dashboard_up Dashboard service up (1 = running)",
    "# TYPE vigilantai_dashboard_up gauge",
    "vigilantai_dashboard_up 1",
    "",
    "# HELP vigilantai_dashboard_uptime_seconds Dashboard uptime in seconds",
    "# TYPE vigilantai_dashboard_uptime_seconds gauge",
    `vigilantai_dashboard_uptime_seconds ${uptimeSeconds}`,
    "",
    "# HELP vigilantai_dashboard_build_info Dashboard build information",
    "# TYPE vigilantai_dashboard_info gauge",
    'vigilantai_dashboard_info{version="1.0.0"} 1',
    "",
  ].join("\n");

  return new NextResponse(metrics, {
    status: 200,
    headers: {
      "Content-Type": "text/plain; version=0.0.4; charset=utf-8",
    },
  });
}
