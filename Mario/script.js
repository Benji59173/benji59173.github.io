  var password = prompt("Entrez le mot de passe :");

  if (password == "memes") {
    alert("Bienvenue sur Super Mario 64 !");
    document.querySelector("body").style.display = "block";
  } else {
    alert("Mot de passe Incorect !");
  }