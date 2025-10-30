// === FUNCIONALIDAD WALLET ===
const btnActualizarWallet = document.getElementById("btnActualizarWallet");
const walletList = document.getElementById("walletList");

if (btnActualizarWallet) {
  btnActualizarWallet.addEventListener("click", () => {
    // Simulación de actualización de datos
    const tokensExtra = Math.floor(Math.random() * 5) + 1;
    const totalPesos = (723.8 + tokensExtra * 10).toFixed(2);

    walletList.innerHTML = `
      <div class="wallet-card">
        <h3>Pesos mexicanos:</h3>
        <p>$${totalPesos}</p>
      </div>
      <div class="wallet-card">
        <h3>Ajolote:</h3>
        <p>${3 + tokensExtra} tokens</p>
      </div>
      <div class="wallet-card">
        <h3>Jaguar:</h3>
        <p>${1 + tokensExtra} tokens</p>
      </div>
      <div class="wallet-card">
        <h3>Mariposa Monarca:</h3>
        <p>${6 + tokensExtra} tokens</p>
      </div>
      <div class="wallet-card">
        <h3>Águila:</h3>
        <p>${1 + tokensExtra} tokens</p>
      </div>
    `;

    alert("✅ Saldos actualizados con éxito.");
  });
}
