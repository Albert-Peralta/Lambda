// Actualizar tokens simuladamente
let tokens = 17;

document.querySelectorAll('.op-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    if (btn.textContent.includes("Agregar")) {
      tokens += 1;
      document.getElementById('tokenCount').textContent = `${tokens} Tokens`;
      alert("Has ganado 1 token por una nueva acción 🪙");
    } else {
      alert(`Función "${btn.textContent.trim()}" próximamente disponible 🚀`);
    }
  });
});
