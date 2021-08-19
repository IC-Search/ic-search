import { search } from "../../declarations/search";

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const name = document.getElementById("name").value.toString();
  // Interact with search actor, calling the greet method
  const greeting = await search.greet(name);

  document.getElementById("greeting").innerText = greeting;
});
