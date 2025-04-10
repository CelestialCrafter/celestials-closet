// based on https://github.com/adryd325/oneko.js

const speed = 10;
const max = 15;
const updateInterval = 100;
const spriteSets = {
  idle: [[-3, -3]],
  alert: [[-7, -3]],
  scratchSelf: [
    [-5, 0],
    [-6, 0],
    [-7, 0],
  ],
  scratchWallN: [
    [0, 0],
    [0, -1],
  ],
  scratchWallS: [
    [-7, -1],
    [-6, -2],
  ],
  scratchWallE: [
    [-2, -2],
    [-2, -3],
  ],
  scratchWallW: [
    [-4, 0],
    [-4, -1],
  ],
  tired: [[-3, -2]],
  sleeping: [
    [-2, 0],
    [-2, -1],
  ],
  N: [
    [-1, -2],
    [-1, -3],
  ],
  NE: [
    [0, -2],
    [0, -3],
  ],
  E: [
    [-3, 0],
    [-3, -1],
  ],
  SE: [
    [-5, -1],
    [-5, -2],
  ],
  S: [
    [-6, -3],
    [-7, -2],
  ],
  SW: [
    [-5, -3],
    [-6, -1],
  ],
  W: [
    [-4, -2],
    [-4, -3],
  ],
  NW: [
    [-1, 0],
    [-1, -1],
  ],
};

let mouseX = 0;
let mouseY = 0;
document.addEventListener("mousemove", function (event) {
  mouseX = event.clientX;
  mouseY = event.clientY;
});

const nekos = [];
let frame = 0;
let lastFrame = 0;
const animate = (timestamp) => {
  if (timestamp - lastFrame >= updateInterval) {
    lastFrame = timestamp;
    frame++;

    for (const neko of nekos) {
      neko();
    }
  }

  window.requestAnimationFrame(animate);
};

window.requestAnimationFrame(animate);

const init = (state) => {
  const element = document.createElement("div");
  element.classList.add("oneko");
  element.ariaHidden = true;
  element.style.left = `${state.nekoX - 16}px`;
  element.style.top = `${state.nekoY - 16}px`;
  element.onclick = () => {
    if (nekos.length <= max) oneko();
  };

  document.body.appendChild(element);
  state.element = element;
};

const setSprite = (element) => (dir, frame) => {
	console.log(dir);
  const sprite = spriteSets[dir][frame % spriteSets[dir].length];
  element.style.backgroundPosition = `${sprite[0] * 32}px ${sprite[1] * 32}px`;
};

const resetIdleAnimation = (state) => () => {
  state.idleAnimation = null;
  state.idleAnimationFrame = 0;
};

const idle = (state, setSprite) => () => {
  state.idleTime++;

  // every ~20 seconds
  if (
    state.idleTime > 10 &&
    Math.floor(Math.random() * 200) == 0 &&
    !state.idleAnimation
  ) {
    const { nekoX: x, nekoY: y } = state;
    let available = ["sleeping", "scratchSelf"];
    if (x < 32) available.push("scratchWallW");
    if (y < 32) available.push("scratchWallN");
    if (x > window.innerWidth - 32) available.push("scratchWallE");
    if (y > window.innerHeight - 32) available.push("scratchWallS");

    state.idleAnimation =
      available[Math.floor(Math.random() * available.length)];
  }

  const { idleAnimationFrame: idleFrame } = state;
  switch (state.idleAnimation) {
    case "sleeping":
      if (idleFrame < 8) {
        setSprite("tired", 0);
      } else {
        setSprite("sleeping", Math.floor(idleFrame / 4));
        if (idleFrame > 192) resetIdleAnimation();
      }

      break;
    case "scratchSelf":
      setSprite(state.idleAnimation, frame);
      if (frame > 9) resetIdleAnimation();

      break;
    default:
      setSprite("idle", 0);

      return;
  }

  state.idleAnimationFrame++;
};

const tick = (state, idle, setSprite) => () => {
  const diffX = state.nekoX - mouseX;
  const diffY = state.nekoY - mouseY;
  const distance = Math.sqrt(diffX ** 2 + diffY ** 2);
  if (distance < speed || distance < 48) {
    idle();
    return;
  }

  state.idleAnimation = null;
  state.idleAnimationFrame = 0;
  if (state.idleTime > 1) {
    setSprite("alert", 0);
    // count down after being alerted before moving
    state.idleTime = Math.min(state.idleTime, 7);
    state.idleTime -= 1;
    return;
  }

  let direction;
  direction = diffY / distance > 0.5 ? "N" : "";
  direction += diffY / distance < -0.5 ? "S" : "";
  direction += diffX / distance > 0.5 ? "W" : "";
  direction += diffX / distance < -0.5 ? "E" : "";
  setSprite(direction, frame);

  state.nekoX -= (diffX / distance) * speed;
  state.nekoY -= (diffY / distance) * speed;

  state.nekoX = Math.min(Math.max(16, state.nekoX), window.innerWidth - 16);
  state.nekoY = Math.min(Math.max(16, state.nekoY), window.innerHeight - 16);

  state.element.style.left = `${state.nekoX - 16}px`;
  state.element.style.top = `${state.nekoY - 16}px`;
};

const oneko = () => {
  const state = {
    nekoX: 32,
    nekoY: 32,

    idleTime: 0,
    idleAnimation: null,
    idleAnimationFrame: 0,

    element: null,
  };

  init(state);

  const ss = setSprite(state.element);
  nekos.push(tick(state, idle(state, ss), ss));
};

oneko();
