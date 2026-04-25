async function reloadData() {
    try {
        const response = await fetch(ENDPOINT);
        const newData = await response.json();
        updatePage(newData);
    } catch (error) {
        console.error("Failed to reload data:", error);
    }
}

/**
 * @param {Data} newData
 */
function updatePage(newData) {
    const tempCards = document.querySelectorAll(".stat-card");
    const dataItems = document.querySelectorAll(".data-item");

    if (newData.temperature !== null) {
        tempCards[0].innerHTML = `
            <div class="stat-label">🌡️ Sıcaklık</div>
            <div class="stat-value">${newData.temperature.toFixed(1)}<span class="stat-unit">°C</span></div>
        `;
        dataItems[0].innerHTML = `
            <div class="data-label">🌡️ Sıcaklık</div>
            <div class="data-value">${newData.temperature.toFixed(2)} °C</div>
        `;
    }

    if (newData.humidity !== null) {
        tempCards[1].innerHTML = `
            <div class="stat-label">💧 Nem Oranı</div>
            <div class="stat-value"><span class="stat-unit">%</span>${newData.humidity}</div>
        `;
        dataItems[1].innerHTML = `
            <div class="data-label">💧 Nem Oranı</div>
            <div class="data-value">%${newData.humidity}</div>
        `;
    }

    if (newData.windSpeed1Min !== null) {
        tempCards[2].innerHTML = `
            <div class="stat-label">💨 Rüzgar Hızı</div>
            <div class="stat-value">${newData.windSpeed1Min.toFixed(1)}<span class="stat-unit">m/s</span></div>
        `;
        dataItems[3].innerHTML = `
            <div class="data-label">💨 Rüzgar Hızı (1 dakika)</div>
            <div class="data-value">${newData.windSpeed1Min.toFixed(2)} m/s</div>
        `;
    }

    if (newData.airPressure !== null) {
        tempCards[3].innerHTML = `
            <div class="stat-label">📈 Basınç</div>
            <div class="stat-value">${newData.airPressure.toFixed(0)}<span class="stat-unit">hPa</span></div>
        `;
        dataItems[2].innerHTML = `
            <div class="data-label">📈 Hava Basıncı</div>
            <div class="data-value">${newData.airPressure.toFixed(1)} hPa</div>
        `;
    }

    if (newData.maxWindSpeed5Min !== null) {
        dataItems[4].innerHTML = `
            <div class="data-label">🌪️ Max Rüzgar Hızı (5 dakika)</div>
            <div class="data-value">${newData.maxWindSpeed5Min.toFixed(2)} m/s</div>
        `;
    }

    if (newData.windDirection !== null) {
        dataItems[5].innerHTML = `
            <div class="data-label">🧭 Rüzgar Yönü</div>
            <div class="data-value">${newData.windDirection}°</div>
        `;
    }

    if (newData.rainfall1Hour !== null) {
        dataItems[6].innerHTML = `
            <div class="data-label">🌧️ Yağmur (1 saat)</div>
            <div class="data-value">${newData.rainfall1Hour.toFixed(2)} mm</div>
        `;
    }

    if (newData.rainfall1Day !== null) {
        dataItems[7].innerHTML = `
            <div class="data-label">🌧️ Yağmur (1 gün)</div>
            <div class="data-value">${newData.rainfall1Day.toFixed(2)} mm</div>
        `;
    }
}

setInterval(reloadData, 5000);
