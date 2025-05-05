import init, { get_weather_predictions } from "../pkg/wasm_table_project.js";

async function run() {
  await init();
  const tableContainer = document.getElementById("table-container");

  function showToast(message) {
    const toast = document.createElement("div");
    toast.textContent = message;
    toast.style.position = "fixed";
    toast.style.bottom = "20px";
    toast.style.right = "20px";
    toast.style.backgroundColor = "#333";
    toast.style.color = "#fff";
    toast.style.padding = "10px 20px";
    toast.style.borderRadius = "5px";
    toast.style.boxShadow = "0 2px 5px rgba(0, 0, 0, 0.3)";
    toast.style.zIndex = "1000";
    document.body.appendChild(toast);

    setTimeout(() => {
      toast.remove();
    }, 3000);
  }

  function copyToClipboard(alarmMacro) {
    navigator.clipboard
      .writeText(alarmMacro)
      .then(() => {
        showToast("Copied to clipboard: " + alarmMacro);
      })
      .catch((err) => {
        alert("Failed to copy: ", err);
      });
  }

  function renderWeatherTable() {
    const predictions = get_weather_predictions();
    let tableHtml =
      "<table><tr><th>Upcoming weather</th><th>Server time</th><th>Time until</th><th>Alarm macro</th></tr>";
    predictions.forEach((prediction, index) => {
      tableHtml +=
        `<tr><td>${prediction.weather}</td><td>${prediction.start_time}</td><td>${prediction.time_until}</td>` +
        `<td><button id="copy-button-${index}">Copy</button></td></tr>`;
    });
    tableHtml += "</table>";
    tableContainer.innerHTML = tableHtml;

    // Attach event listeners to buttons
    // since the alarm macros contain double quotes, it's easier/safer to do this in JS rather than
    // trying to escape the quotes in HTML interpolation
    predictions.forEach((prediction, index) => {
      const button = document.getElementById(`copy-button-${index}`);
      button.addEventListener("click", () =>
        copyToClipboard(prediction.alarm_macro)
      );
    });
  }

  renderWeatherTable();
  setInterval(renderWeatherTable, 5000);
}

run();
