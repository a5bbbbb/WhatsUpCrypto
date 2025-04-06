document.getElementById("searchbar").addEventListener("keyup", function(event){
    event.preventDefault();
    if(event.code == "Enter")
        search();
})

async function search() {
    let query = document.getElementById("searchbar").value;
    if(!query)
        return;

    try{
        let res = await fetch("/news?coin=" + query);

        let data = await res.json();

        updateList(data);
    } catch(e) {
        console.log(e);
    }
}

function updateList(data) {
    let cont = document.getElementById("cont");
    cont.innerHTML = "";

    if(data.error){
        cont.innerHTML = `<h4>No article was found.</h4>`;
        return;
    }

    console.log(data, data.totalResults);
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