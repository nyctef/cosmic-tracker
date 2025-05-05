import init, { update_table } from "../pkg/wasm_table_project.js";

async function run() {
  await init();
  const tableContainer = document.getElementById("table-container");
  tableContainer.innerHTML = update_table();

  setInterval(() => {
    tableContainer.innerHTML = update_table();
  }, 5000); // Update table every 5 seconds
}

run();
