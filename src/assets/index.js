document.getElementById("searchbar").addEventListener("keyup", function(event){
    event.preventDefault();
    if(event.code == "Enter")
        search();
});

let currentDisplayCount = 10;
let allArticles = []; 

function search() {
    let spinner = document.getElementById("loader");
    currentDisplayCount = 10;

    spinner.classList.toggle("invis");

    let query = document.getElementById("searchbar").value;
    if(!query)
        return;

    fetch("/news?coin=" + query)
    .then(async res => {
        await handleErrors(res); 
        return await res.json();
    })
    .then(data => {
        allArticles = data.articles;
        updateList(data, currentDisplayCount);
    })
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

function updateList(data, displayCount) {
    let cont = document.getElementById("cont");
    cont.innerHTML = "";

    if (!data.articles || data.articles.length === 0) {
        cont.innerHTML = `<h4 class="text-center my-4">No articles found.</h4>`;
        return;
    }

    let articlesHTML = `
        <div class="container mb-4">
            <h4 class="text-center">Found ${data.totalResults} article(s):</h4>
            <div class="progress mt-2" role="progressbar" aria-label="Articles loaded">
                <div class="progress-bar" style="width: ${Math.min(100, (displayCount / data.totalResults) * 100)}%"></div>
            </div>
            <p class="text-center text-muted mt-1">Showing ${Math.min(displayCount, data.totalResults)} of ${data.totalResults} articles</p>
        </div>
    `;

    const articlesToShow = Math.min(displayCount, data.totalResults);
    
    for (let i = 0; i < articlesToShow; i++) {
        const article = data.articles[i];
        if (!article) break;
        
        const publishDate = new Date(article.publishedAt);
        const formattedDate = publishDate.toLocaleDateString('en-US', { 
            year: 'numeric', 
            month: 'short', 
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit'
        });
        
        articlesHTML += `
        <div class="container border border-primary-subtle rounded-4 p-3 m-2 article-card">
            <div class="row">
                <div class="col-md-12">
                    <a href="${article.url}" class="text-decoration-none" target="_blank">
                        <h3 class="article-title">${article.title}</h3>
                    </a>
                    <div class="d-flex justify-content-between flex-wrap text-muted mb-3">
                        <h6 class="d-inline">Source: ${article.author || article.source?.name || "Unknown"}</h6>
                        <h6 class="d-inline">📅 ${formattedDate}</h6>
                    </div>
                    <p class="fs-5 article-content">
                        ${article.content || article.description || "No content available"}
                    </p>
                    <div class="text-end">
                        <a href="${article.url}" class="btn btn-sm btn-outline-primary" target="_blank">Read more</a>
                    </div>
                </div>
            </div>
        </div>
        `;
    }
    
    if (displayCount < data.totalResults) {
        articlesHTML += `
        <div class="text-center my-4">
            <button id="loadMoreBtn" class="btn btn-primary">
                Load More Articles
            </button>
        </div>
        `;
    }
    
    cont.innerHTML = articlesHTML;
    
    const loadMoreBtn = document.getElementById("loadMoreBtn");
    if (loadMoreBtn) {
        loadMoreBtn.addEventListener("click", () => {
            currentDisplayCount += 10;
            updateList({ articles: allArticles, totalResults: data.totalResults }, currentDisplayCount);
        });
    }
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