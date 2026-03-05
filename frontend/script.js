const API_URL = "http://127.0.0.1:3000/api";

async function loadMenu() {
    try {
        const response = await fetch(`${API_URL}/menu`);
        const data = await response.json();

        const list = document.getElementById("list-menu");
        list.innerHTML = "";

        const token = localStorage.getItem("token_kasir");

        data.forEach(menu => {
            const grupTombol = token
                ? `<div style="display: flex; gap: 5px;">
                           <button onclick="editMenu(${menu.id}, '${menu.nama}', ${menu.harga})" style="width: auto; padding: 6px 10px; background-color: #ffc107; color: black; border: 1px solid #e0a800; font-size: 12px;">✏️</button>
                           <button onclick="hapusMenu(${menu.id})" style="width: auto; padding: 6px 10px; background-color: #dc3545; font-size: 12px;">❌</button>
                       </div>`
                : "";

            list.innerHTML += `<li style="align-items: center;">
                    <span><strong>${menu.nama}</strong> - Rp${menu.harga}</span>
                    ${grupTombol}
                </li>`;
        });
    } catch (error) {
        console.error("Gagal mengambil menu", error);
    }
}

async function login() {
    const user = document.getElementById("username").value;
    const pass = document.getElementById("password").value;
    const pesanError = document.getElementById("login-error");

    try {
        const response = await fetch(`${API_URL}/login`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ username: user, password: pass })
        });

        const data = await response.json();

        if (response.ok) {
            localStorage.setItem("token_kasir", data.token);
            cekStatusLogin();
            await loadMenu();
        } else {
            pesanError.innerText = data.error || "Login gagal!";
        }
    } catch (error) {
        pesanError.innerText = "Server tidak merespons!";
    }
}

async function tambahMenu() {
    const nama = document.getElementById("nama-menu").value;
    const harga = parseInt(document.getElementById("harga-menu").value);
    const pesanForm = document.getElementById("form-pesan");
    const token = localStorage.getItem("token_kasir");

    try {
        const response = await fetch(`${API_URL}/menu`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `Bearer ${token}`
            },
            body: JSON.stringify({ nama: nama, harga: harga })
        });

        const data = await response.json();

        if (response.ok) {
            pesanForm.style.color = "green";
            pesanForm.innerText = "Berhasil ditambahkan!";
            await loadMenu();
        } else {
            pesanForm.style.color = "red";
            pesanForm.innerText = data.error;
        }
    } catch (error) {
        console.error(error);
    }
}

async function hapusMenu(id) {
    if (!confirm("Apakah Anda yakin ingin menghapus menu ini?")) {
        return;
    }

    const token = localStorage.getItem("token_kasir");
    if (!token) {
        alert("Anda harus login terlebih dahulu!");
        return;
    }

    try {
        const response = await fetch(`${API_URL}/menu/${id}`, {
            method: "DELETE",
            headers: {
                "Authorization": `Bearer ${token}`
            }
        });

        if (response.ok) {
            await loadMenu();
        } else {
            const data = await response.json();
            alert("Gagal menghapus: " + data.error);
        }
    } catch (error) {
        console.error(error);
        alert("Gagal menghubungi server.");
    }
}

async function editMenu(id, namaSaatIni, hargaSaatIni) {
    const token = localStorage.getItem("token_kasir");
    if (!token) {
        alert("Anda harus login terlebih dahulu!");
        return;
    }

    const inputHarga = prompt(`Ubah harga untuk "${namaSaatIni}":`, hargaSaatIni);

    if (inputHarga === null || inputHarga.trim() === "") {
        return;
    }

    const hargaBaru = parseInt(inputHarga);

    if (isNaN(hargaBaru) || hargaBaru < 0) {
        alert("Harga tidak valid! Harus berupa angka dan tidak boleh minus.");
        return;
    }

    try {
        const response = await fetch(`${API_URL}/menu/${id}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `Bearer ${token}`
            },
            body: JSON.stringify({ nama: namaSaatIni, harga: hargaBaru })
        });

        if (response.ok) {
            await loadMenu();
        } else {
            const data = await response.json();
            alert("Gagal mengubah data: " + data.error);
        }
    } catch (error) {
        console.error(error);
        alert("Gagal menghubungi server.");
    }
}

function logout() {
    localStorage.removeItem("token_kasir");
    cekStatusLogin();
    loadMenu();
}

function cekStatusLogin() {
    const token = localStorage.getItem("token_kasir");
    if (token) {
        document.getElementById("login-section").classList.add("hidden");
        document.getElementById("dashboard-section").classList.remove("hidden");
    } else {
        document.getElementById("login-section").classList.remove("hidden");
        document.getElementById("dashboard-section").classList.add("hidden");
    }
}

cekStatusLogin();
loadMenu();