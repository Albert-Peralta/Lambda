document.getElementById('loginForm').addEventListener('submit', function (e) {
  e.preventDefault();

  const email = document.getElementById('email').value.trim();
  const password = document.getElementById('password').value.trim();

  if (email === "" || password === "") {
    alert("Por favor, completa todos los campos.");
    return;
  }

  // Simulación de login (aquí luego se conectará con Rust/Soroban)
  if (email === "demo@lambda.com" && password === "12345") {
    alert("Inicio de sesión exitoso");
    window.location.href = "home.html"; // redirige a la siguiente página
  } else {
    alert("Credenciales incorrectas. Intenta nuevamente.");
  }
});
