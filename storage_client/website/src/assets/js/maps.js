var map = L.map('map').setView([0, 0], 13); // Replace [0, 0] with the initial coordinates and 13 with the initial zoom level
L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    maxZoom: 19,
    attribution: '© <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
}).addTo(map);

function updateMap(address) {
    var url = 'https://nominatim.openstreetmap.org/search?format=json&q=' + address;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            if (data.length > 0) {
                var lat = data[0].lat;
                var lon = data[0].lon;
                var zoom = 16;
                map.setView([lat, lon], zoom); // Update the map's view to the coordinates of the address
                L.marker([lat, lon]).addTo(map); // Add a marker at the address
            }
        });
}