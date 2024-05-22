// Is this the best way to do the swap?
function setColorscheme(event) {
  // `detail.fragment` is not documented for oobBeforeSwap. Is this stable?
  const swapee = event.detail.fragment.firstChild;

  if (!swapee.hasChildNodes()) {
    event.preventDefault();
    //   // // Only swap the style.
    //   // // event.detail.shouldSwap = false;

    const style = event.detail.fragment.firstChild.getAttribute("style");
    event.detail.target.setAttribute("style", style);
  }
}

let configChangedByUser = false;

htmx.on("body", "htmx:beforeSwap", function(ev) { 
  console.warn(ev);
  // ev.detail.shouldSwap = false;
});

document
  .querySelector("textarea[name=configuration]")
  .addEventListener("input", () => {
    configChangedByUser = true;
  });
