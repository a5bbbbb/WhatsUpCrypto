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
 * Handler of errors for queries to the server.
 * @param {Response} res 
 * @throws for 4xx and 5xx responses with message in Error object. 
 */
async function handleErrors(res) {

    let copy = res.clone();

    if(res.status == 400) {
        console.log("Bad request. 400 bad request");
        throw new Error((await res.json()).error || (await copy.text()));
    }

    if(res.status == 429) {
        console.log("Rate limit exceeded. 429 too many requests");
        throw new Error("You submit requests too often. Please try again in few minutes.");
    }

    if(res.status >= 500) {
        console.log("Server error.");
        throw new Error("Something is wrong with our server. " + (await res.text()).error);
    }

    if(res.status >= 400) {
        console.log("Bad request. 4xx code.");
        throw new Error("Something is wrong with your requests. " + (await res.text()).error);
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
    toast.classList.add("my-2");
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