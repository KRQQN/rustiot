window.createChart = function (element, options) {
  const chart = echarts.init(element);
  chart.setOption(options);
  return chart;
};

window.updateChart = function (chart, options) {
  chart.setOption(options);
};

window.resizeChart = function (chart) {
  chart.resize();
};

window.disposeChart = function (chart) {
  chart.dispose();
};
