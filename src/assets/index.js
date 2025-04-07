document.getElementById("searchbar").addEventListener("keyup", function(event){
    event.preventDefault();
    if(event.code == "Enter")
        search();
})

function search() {
    let spinner = document.getElementById("loader");

    spinner.classList.toggle("invis");

    let query = document.getElementById("searchbar").value;
    if(!query)
        return;

    fetch("/news?coin=" + query)
    .then(async res => {
        await handleErrors(res); 
        return await res.json();
    })
    .then(data => updateList(data))
    .catch(err => makeToast("Error", err.toString()))
    .finally(() => spinner.classList.toggle("invis"));
}
/**
 * 
 * @param {Response} res 
 * @returns 
 */
async function handleErrors(res) {
    if(res.status >= 500) {
        console.log("Server error.")
        throw new Error("Something is wrong with our server. " + (await res.json()).error)
    }

    if(res.status >= 400) {
        console.log("Bad request.")
        throw new Error("Something is wrong with your requests. " + (await res.json()).error)
    }
}

function updateList(data) {

    let cont = document.getElementById("cont");
    cont.innerHTML = "";
    cont.innerHTML = `<h4>No article was found.</h4>`;

    let articlesHTML = `<h4>Found ${data.totalResults} article(s):</h4>`;

    for(let i = 0; i < data.totalResults; i++) {
        let article = data.articles[i];
        if(!article)
            break;
        console.log(i, data.articles[i]);
        articlesHTML = articlesHTML + "\n" +
        `
        <div class="container border border-primary-subtle rounded-4 p-3 m-2">
            <a href="${article.url}"><h3>${article.title}</h3></a>
            <div class="d-flex justify-content-between flex-wrap">
                <h4 class="d-inline">Source: ${article.author}</h4>
                <h4 class="d-inline">Date: ${(new Date(article.publishedAt)).toDateString()}</h4>
            </div>
            <p class="fs-5">
                ${article.content}
            </p>
        </div>
        `
    }
    cont.innerHTML = articlesHTML;
}

async function makeToast(subject, content, colorScheme){
    if(!colorScheme) 
        colorScheme = "text-bg-success";
    if(subject == "Error")
        colorScheme = "text-bg-danger";
    const toastContainer = document.getElementById("toasts");
    const toast = document.createElement("div");
    toast.classList.add("toast");
    toast.classList.add(colorScheme);
    toast.setAttribute("role", "alert");
    toast.setAttribute("aria-live", "assertive");
    toast.setAttribute("aria-atomic", "true");
    toast.innerHTML = `
        <div class="toast-header">
        <strong class="me-auto">${subject}</strong>
        <small></small>
        <button type="button" class="btn-close" data-bs-dismiss="toast" aria-label="Close"></button>
        </div>
        <div class="toast-body">
        ${content}
        </div>
    `;
    toastContainer.appendChild(toast);
    const toastBootstrap = bootstrap.Toast.getOrCreateInstance(toast);
    toastBootstrap.show();
}