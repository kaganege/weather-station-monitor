/* Expands to the endpoint */
const ENDPOINT = new URL("{{ endpoint }}");

/**
 * @param {number} value
 * @param {number} decimals
 */
const round = (value, decimals) => {
    const factor = 10 ** decimals;
    return Math.round(value * factor) / factor;
};

const el = {
    temp: document.querySelector(".js-temp"),
    humidity: document.querySelector(".js-humidity"),
    windSpeed: document.querySelector(".js-wind-speed"),
    pressure: document.querySelector(".js-pressure"),

    tempDetail: document.querySelector(".js-temp-detail"),
    humidityDetail: document.querySelector(".js-humidity-detail"),
    pressureDetail: document.querySelector(".js-pressure-detail"),
    windSpeedDetail: document.querySelector(".js-wind-speed-detail"),

    maxWind: document.querySelector(".js-max-wind"),
    windDir: document.querySelector(".js-wind-dir"),
    rain1h: document.querySelector(".js-rain-1h"),
    rain1d: document.querySelector(".js-rain-1d"),
};

const windMap = {
    north: "Kuzey",
    "north-east": "Kuzey Doğu",
    "north-west": "Kuzey Batı",
    east: "Doğu",
    west: "Batı",
    south: "Güney",
    "south-west": "Güney Batı",
    "south-east": "Güney Doğu",
};

/**
 * @param {Element | null} node
 * @param {any} value
 * @param {boolean} available
 */
const setText = (node, value, available) => {
    if (!node) return;
    const str = String(value);
    if (node.textContent !== str) {
        node.textContent = str;
    }

    node.classList.toggle("unavailable", !available);
    node.closest(".stat-value")?.classList.toggle("unavailable", !available);
};

async function reloadData() {
    try {
        const res = await fetch(ENDPOINT);
        const d = await res.json();
        update(d);
    } catch (err) {
        console.error("Failed to reload data:", err);
    }
}

/**
 * @param {Data} d
 */
function update(d) {
    if (d.temperature != null) {
        setText(el.temp, round(d.temperature, 1), true);
        setText(el.tempDetail, `${round(d.temperature, 2)} °C`, true);
    } else {
        setText(el.temp, "-", false);
        setText(el.tempDetail, "Veri yok", false);
    }

    if (d.humidity != null) {
        setText(el.humidity, d.humidity, true);
        setText(el.humidityDetail, `%${d.humidity}`, true);
    } else {
        setText(el.humidity, "-", false);
        setText(el.humidityDetail, "Veri yok", false);
    }

    if (d.windSpeed1Min != null) {
        setText(el.windSpeed, round(d.windSpeed1Min, 1), true);
        setText(el.windSpeedDetail, `${round(d.windSpeed1Min, 2)} m/s`, true);
    } else {
        setText(el.windSpeed, "-", false);
        setText(el.windSpeedDetail, "Veri yok", false);
    }

    if (d.airPressure != null) {
        setText(el.pressure, Math.round(d.airPressure), true);
        setText(el.pressureDetail, `${round(d.airPressure, 1)} hPa`, true);
    } else {
        setText(el.pressure, "-", false);
        setText(el.pressureDetail, "Veri yok", false);
    }

    if (d.maxWindSpeed5Min != null) {
        setText(el.maxWind, `${round(d.maxWindSpeed5Min, 2)} m/s`, true);
    } else {
        setText(el.maxWind, "Veri yok", false);
    }

    if (d.windDirection != null) {
        setText(el.windDir, windMap[d.windDirection], true);
    } else {
        setText(el.windDir, "Veri yok", false);
    }

    if (d.rainfall1Hour != null) {
        setText(el.rain1h, `${round(d.rainfall1Hour, 2)} mm`, true);
    } else {
        setText(el.rain1h, "Veri yok", false);
    }

    if (d.rainfall1Day != null) {
        setText(el.rain1d, `${round(d.rainfall1Day, 2)} mm`, true);
    } else {
        setText(el.rain1d, "Veri yok", false);
    }
}

setInterval(reloadData, 5000);
