import init, { get_table_data } from "../pkg/wasm_table_project.js";

async function run() {
  await init();
  const tableContainer = document.getElementById("table-container");

  function renderTable() {
    const data = get_table_data();
    let tableHtml = "<table>";
    data.forEach((row) => {
      tableHtml += "<tr>";
      row.cells.forEach((cell) => {
        tableHtml += `<td>${cell}</td>`;
      });
      tableHtml += "</tr>";
    });
    tableHtml += "</table>";
    tableContainer.innerHTML = tableHtml;
  }

  renderTable();
  setInterval(renderTable, 5000);
}

run();
