import init, { get_weather_predictions } from "../pkg/wasm_table_project.js";

async function run() {
  await init();
  const tableContainer = document.getElementById("table-container");

  function renderWeatherTable() {
    const predictions = get_weather_predictions();
    let tableHtml =
      "<table><tr><th>Weather</th><th>Start Time</th><th>Time Until</th></tr>";
    predictions.forEach((prediction) => {
      tableHtml += `<tr><td>${prediction.weather}</td><td>${prediction.start_time}</td><td>${prediction.time_until}</td></tr>`;
    });
    tableHtml += "</table>";
    tableContainer.innerHTML = tableHtml;
  }

  renderWeatherTable();
  setInterval(renderWeatherTable, 5000);
}

run();
