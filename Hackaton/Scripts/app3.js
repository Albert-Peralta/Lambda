// === FUNCIONALIDAD DE HISTORIAL ===
const historialList = document.getElementById("historial-list");
const btnAgregarHistorial = document.getElementById("btnAgregarHistorial");

if (btnAgregarHistorial) {
  btnAgregarHistorial.addEventListener("click", () => {
    const fechaActual = new Date();
    const fechaFormateada = fechaActual.toLocaleDateString("es-MX");
    const nuevaCiudad = prompt("¿En qué ciudad realizaste la recolección?");
    if (!nuevaCiudad) return;

    const nuevaCard = document.createElement("div");
    nuevaCard.classList.add("historial-card");
    nuevaCard.innerHTML = `
      <h3>Recolección de token</h3>
      <p>${fechaFormateada}, ${nuevaCiudad}</p>
    `;

    historialList.prepend(nuevaCard);
  });
}
