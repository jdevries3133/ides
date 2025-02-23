function makeId(length) {
  let result = "";
  const characters =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  const charactersLength = characters.length;
  let counter = 0;
  while (counter < length) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
    counter += 1;
  }
  return result;
}

htmx.on("htmx:responseError", () => {
  const el = document.createElement("p");
  const id = makeId(20);
  el.id = id;
  el.innerText =
    "An error occurred; sorry for the inconvenience! (click to dismiss)";
  el.classList.add("bg-red-100");
  el.classList.add("p-2");
  el.classList.add("rounded");
  el.classList.add("w-full");
  el.classList.add("sticky");
  el.classList.add("top-0");
  el.classList.add("dark:text-black");
  el.classList.add("cursor-pointer");
  document.body.insertBefore(el, document.body.firstChild);
  el.addEventListener("click", () => {
    document.getElementById(id).remove();
  });
});

htmx.on("htmx:beforeSwap", (e) => {
  if (e.detail.xhr.status === 400) {
    e.detail.shouldSwap = true;
    e.detail.isError = false;
  }
});

htmx.config.defaultSwapStyle = "outerHTML";

function transformBookLinks(currentArea) {
  for (const attribute of ["hx-get", "href"]) {
    for (const element of document.querySelectorAll(
      `[${attribute}^='/book']`,
    )) {
      const url = new URL(
        window.location.origin + element.getAttribute(attribute),
      );
      url.searchParams.set("screen_area", currentArea);
      element.setAttribute(attribute, url.toString());
    }
  }
}

function navigateOnScreenSizeChange(currentArea) {
  const params = new URLSearchParams(window.location.search);
  const paramArea = parseInt(params.get("screen_area"));
  if (
    window.location.pathname == "/book" &&
    (isNaN(paramArea) || paramArea !== currentArea)
  ) {
    params.set("screen_area", currentArea);
    window.location.search = params.toString();
  }
}

/**
 * Book links need to include the screen size, which is accompished by this
 * client-side script
 */
function setScreenSize() {
  const currentArea = window.innerWidth * window.innerHeight;

  // Do this first because if we navigate, then we'll never run the second
  // function anyway.
  navigateOnScreenSizeChange(currentArea);
  transformBookLinks(currentArea);
}

htmx.on("htmx:afterSwap", setScreenSize);
window.addEventListener("DOMContentLoaded", setScreenSize);

let debounce;
window.addEventListener("resize", () => {
  let later = () => {
    debounce = null;
    setScreenSize();
  };
  clearTimeout(debounce);
  debounce = setTimeout(later, 200);
});
