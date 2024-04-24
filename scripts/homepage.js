function togglePurchaseSection(type) {
    // 0 -> buy
    // 1 -> sell

    var headers = document.querySelectorAll("#purchase #header h5");
    headers.forEach(e => {
        e.ariaLabel = "";
    })
    headers[type].ariaLabel = "selected";
}

function togglePurchaseSortSection(type) {
    // 0 -> amount
    // 1 -> price
    // 2 -> total

    var headers = document.querySelectorAll("#purchase #filters #sort > div h5");
    headers.forEach((e, i) => {
        // definitely smarter way to do this
        if (i == type) {
            if (e.ariaLabel == "selected") {
                e.ariaLabel = "";
            } else {
                e.ariaLabel = "selected";
            }
        } else {
            e.ariaLabel = "";
        }
    })
}

function togglePurchaseSortOrder(e) {
    e.ariaLabel = e.ariaLabel == "asc" ? "desc" : "asc";
}
