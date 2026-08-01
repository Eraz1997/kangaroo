function goToUser() {
    const username = document.getElementById("username").value;
    window.location.replace(`/users/${username}`);
}
