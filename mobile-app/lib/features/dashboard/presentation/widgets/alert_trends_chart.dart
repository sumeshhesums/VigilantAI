import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';

class AlertTrendsChart extends StatelessWidget {
  final List<FlSpot> criticalSpots;
  final List<FlSpot> highSpots;
  final List<FlSpot> mediumSpots;
  final List<FlSpot> lowSpots;

  const AlertTrendsChart({
    super.key,
    required this.criticalSpots,
    required this.highSpots,
    required this.mediumSpots,
    required this.lowSpots,
  });

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 200,
      child: LineChart(
        LineChartData(
          gridData: FlGridData(
            show: true,
            drawVerticalLine: false,
            horizontalInterval: 1,
            getDrawingHorizontalLine: (value) => FlLine(
              color: Colors.grey.withAlpha(50),
              strokeWidth: 1,
            ),
          ),
          titlesData: const FlTitlesData(show: false),
          borderData: FlBorderData(show: false),
          lineBarsData: [
            _buildLineData('Critical', criticalSpots, Colors.red, 2),
            _buildLineData('High', highSpots, Colors.orange, 2),
            _buildLineData('Medium', mediumSpots, Colors.amber, 2),
            _buildLineData('Low', lowSpots, Colors.green, 2),
          ],
        ),
      ),
    );
  }

  LineChartBarData _buildLineData(
    String label,
    List<FlSpot> spots,
    Color color,
    double width,
  ) {
    return LineChartBarData(
      spots: spots,
      isCurved: true,
      color: color,
      barWidth: width,
      dotData: const FlDotData(show: false),
      belowBarData: BarAreaData(show: false),
    );
  }
}
