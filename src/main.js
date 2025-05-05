import init, { get_weather_predictions } from '../pkg/wasm_table_project.js';

async function run() {
    await init();
    const tableContainer = document.getElementById("table-container");

    function copyToClipboard(alarmMacro) {
        navigator.clipboard.writeText(alarmMacro).then(() => {
            alert("Copied to clipboard: " + alarmMacro);
        }).catch(err => {
            console.error("Failed to copy: ", err);
        });
    }

    function renderWeatherTable() {
        const predictions = get_weather_predictions();
        let tableHtml = "<table><tr><th>Weather</th><th>Start Time</th><th>Time Until</th><th>Action</th></tr>";
        predictions.forEach((prediction, index) => {
            tableHtml += `<tr><td>${prediction.weather}</td><td>${prediction.start_time}</td><td>${prediction.time_until}</td>` +
                         `<td><button id="copy-button-${index}">Copy Alarm</button></td></tr>`;
        });
        tableHtml += "</table>";
        tableContainer.innerHTML = tableHtml;

        // Attach event listeners to buttons
        predictions.forEach((prediction, index) => {
            const button = document.getElementById(`copy-button-${index}`);
            button.addEventListener("click", () => copyToClipboard(prediction.alarm_macro));
        });
    }

    renderWeatherTable();
    setInterval(renderWeatherTable, 5000);
}

run();
