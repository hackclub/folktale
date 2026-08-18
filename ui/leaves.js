const leaves_paths = ["resources/leave_1.png", "resources/leave_2.png", "resources/leave_3.png"]
const leaves_container = document.querySelector("#leaves_container")

let leaves_list = []

const SPAWN_INTERVAL = 500
const MAX_SIZE = 80
const MAX_COUNT = 40
const SPAWN_HEIGHT = -MAX_SIZE
const INITIAL_LEAVES = 18

function leaf_width(leaf) {
    return Math.min(MAX_SIZE, (window.innerWidth / 10) * leaf.size_factor)
}

function apply_size(leaf) {
    leaf.element.style.width = `${leaf_width(leaf)}px`
}

function spawn_leaf(y = SPAWN_HEIGHT) {
    if (leaves_list.length > MAX_COUNT) {
        return;
    }
    const element = document.createElement("img")
    element.className = "leaf leaf-fg"
    if (Math.random() > 0.5) {
        element.className = "leaf leaf-bg"
    }
    element.alt = ""
    element.src = leaves_paths[Math.floor(Math.random() * leaves_paths.length)]

    const leaf = {
        element,
        x_ratio: Math.random(),
        y,
        size_factor: Math.max(Math.sqrt(Math.random()), 0.1),
        time: 0,
        rotation: Math.random() * 360,
        spin: (Math.random() - 0.5) * 60,
        fall_speed: 50 + Math.random() * 50,
        sway_amplitude: 30 + Math.random() * 60,
        sway_speed: 0.6 + Math.random() * 0.8,
        sway_offset: Math.random() * Math.PI * 2,
    }

    leaves_container.appendChild(element)
    apply_size(leaf)
    leaves_list.push(leaf)
}

function update(leaf, delta) {
    leaf.time += delta
    leaf.y += leaf.fall_speed * delta
    leaf.rotation += leaf.spin * delta

    const sway = Math.sin(leaf.time * leaf.sway_speed + leaf.sway_offset) * leaf.sway_amplitude
    const x = leaf.x_ratio * leaves_container.clientWidth
    leaf.element.style.transform =
        `translate(${x + sway}px, ${leaf.y}px) rotate(${leaf.rotation}deg)`
}

let previous_time = performance.now()

function tick(now) {
    const delta = Math.min((now - previous_time) / 1000, 0.1)
    previous_time = now

    const cull_height = leaves_container.clientHeight

    leaves_list = leaves_list.filter(leaf => {
        update(leaf, delta)

        if (leaf.y > cull_height) {
            leaf.element.remove()
            return false
        }

        return true
    })

    requestAnimationFrame(tick)
}

for (let i = 0; i < INITIAL_LEAVES; i++) {
    spawn_leaf(Math.random() * leaves_container.clientHeight)
}

window.addEventListener("resize", () => leaves_list.forEach(apply_size))

setInterval(() => spawn_leaf(), SPAWN_INTERVAL)
requestAnimationFrame(tick)
