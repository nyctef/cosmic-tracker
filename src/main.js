import init, {
  get_weather_predictions,
  get_time_data,
  get_mission_schedule,
} from "../pkg/wasm_table_project.js";

async function run() {
  await init();
  const tableContainer = document.getElementById("table-container");
  const utcTimeElement = document.getElementById("utc-time");
  const localTimeElement = document.getElementById("local-time");
  const eorzeanTimeElement = document.getElementById("eorzean-time");

  function updateTime() {
    const timeData = get_time_data();
    utcTimeElement.textContent = `UTC Time: ${timeData.utc_time}`;
    localTimeElement.textContent = `Local Time: ${timeData.local_time}`;
    eorzeanTimeElement.textContent = `Eorzean Time: ${timeData.eorzean_time}`;
  }

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
      "<table><tr><th>Upcoming weather</th><th>Server time</th><th>Local time</th><th>Time until</th><th>Alarm macro</th></tr>";
    predictions.forEach((prediction, index) => {
      tableHtml +=
        `<tr><td>${prediction.weather}</td><td>${prediction.start_time}</td><td>${prediction.local_time}</td><td>${prediction.time_until}</td>` +
        `<td><button id="copy-button-${index}">Copy Alarm</button></td></tr>`;
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

  async function updateMissionSchedule() {
    const schedule = get_mission_schedule();

    const tbody = document.querySelector("#mission-schedule tbody");
    tbody.innerHTML = schedule
      .map(
        (mission) => `
          <tr>
            <td>${mission.class_name}</td>
            <td>${mission.missions.join(", ")}</td>
            <td>${mission.time_period}</td>
            <td>${mission.interval_until}</td>
            <td>${mission.next_local_time}</td>
          </tr>
        `
      )
      .join("");
  }

  updateTime();
  setInterval(updateTime, 1000);
  renderWeatherTable();
  setInterval(renderWeatherTable, 1000);
  updateMissionSchedule();
  setInterval(updateMissionSchedule, 1000);
}

run();
