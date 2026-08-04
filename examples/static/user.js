user_data = JSON.parse(document.getElementById("kangaroo-data")?.textContent ?? null);
if (user_data.message) {
    document.getElementById("error").innerText = user_data.message;
} else {
    document.getElementById("username").innerText = user_data.username;
    document.getElementById("age").innerText = user_data.age;
    document.getElementById("active").innerText = user_data.active;
}
