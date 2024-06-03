console.warn("start script");

let dialog = document.getElementById("dialog");

dialog.addEventListener("opened", () => {
  console.warn("opened");
  dialog.open = undefined;
});

dialog.addEventListener("cancel", () => {
  console.warn("canceled");
  dialog.open = false;
});
