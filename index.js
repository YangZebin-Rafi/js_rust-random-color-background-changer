const rust = import("./random_color");

rust.then(module => {
  const button = document.createElement("button");
  button.innerHTML = "Change Background Color";
  button.onclick = async () => {
    const color = await module.generate_color();
    document.body.style.backgroundColor = color;
  };
  document.body.appendChild(button);
});
