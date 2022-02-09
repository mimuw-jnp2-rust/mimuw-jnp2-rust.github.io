function initToggleMenu() {
  const $menu = document.querySelector(".menu");
  const $menuIcon = document.querySelector(".menu-icon");
  const $page = document.querySelector(".page");
  $menuIcon.addEventListener("click", () => {
    $menu.classList.toggle("menu-hidden");
    $page.classList.toggle("page-without-menu");
  });
}

function debounce(func, wait) {
  let timeout;

  return () => {
    const context = this;
    const args = arguments;
    clearTimeout(timeout);

    timeout = setTimeout(() => {
      timeout = null;
      func.apply(context, args);
    }, wait);
  };
}

// Taken from mdbook
// The strategy is as follows:
// First, assign a value to each word in the document:
//  Words that correspond to search terms (stemmer aware): 40
//  Normal words: 2
//  First word in a sentence: 8
// Then use a sliding window with a constant number of words and count the
// sum of the values of the words within the window. Then use the window that got the
// maximum sum. If there are multiple maximas, then get the last one.
// Enclose the terms in <b>.
function makeTeaser(body, terms) {
  const TERM_WEIGHT = 40;
  const NORMAL_WORD_WEIGHT = 2;
  const FIRST_WORD_WEIGHT = 8;
  const TEASER_MAX_WORDS = 30;

  const stemmedTerms = terms.map(w => elasticlunr.stemmer(w.toLowerCase()));
  let termFound = false;
  let index = 0;
  let weighted = []; // contains elements of ["word", weight, index_in_document]

  // split in sentences, then words
  let sentences = body.toLowerCase().split(". ");

  for (let i in sentences) {
    let words = sentences[i].split(" ");
    let value = FIRST_WORD_WEIGHT;

    for (let j in words) {
      let word = words[j];

      if (word.length > 0) {
        for (let k in stemmedTerms) {
          if (elasticlunr.stemmer(word).startsWith(stemmedTerms[k])) {
            value = TERM_WEIGHT;
            termFound = true;
          }
        }
        weighted.push([word, value, index]);
        value = NORMAL_WORD_WEIGHT;
      }

      index += word.length;
      index += 1;  // ' ' or '.' if last word in sentence
    }

    index += 1;  // because we split at a two-char boundary '. '
  }

  if (weighted.length === 0) {
    return body;
  }

  let windowWeights = [];
  let windowSize = Math.min(weighted.length, TEASER_MAX_WORDS);
  // We add a window with all the weights first
  let curSum = 0;
  for (let i = 0; i < windowSize; i++) {
    curSum += weighted[i][1];
  }
  windowWeights.push(curSum);

  for (let i = 0; i < weighted.length - windowSize; i++) {
    curSum -= weighted[i][1];
    curSum += weighted[i + windowSize][1];
    windowWeights.push(curSum);
  }

  // If we didn't find the term, just pick the first window
  let maxSumIndex = 0;
  if (termFound) {
    let maxFound = 0;
    // backwards
    for (let i = windowWeights.length - 1; i >= 0; i--) {
      if (windowWeights[i] > maxFound) {
        maxFound = windowWeights[i];
        maxSumIndex = i;
      }
    }
  }

  let teaser = [];
  let startIndex = weighted[maxSumIndex][2];
  for (let i = maxSumIndex; i < maxSumIndex + windowSize; i++) {
    let word = weighted[i];
    if (startIndex < word[2]) {
      // missing text from index to start of `word`
      teaser.push(body.substring(startIndex, word[2]));
      startIndex = word[2];
    }

    // add <em/> around search terms
    if (word[1] === TERM_WEIGHT) {
      teaser.push("<b>");
    }
    startIndex = word[2] + word[0].length;
    teaser.push(body.substring(word[2], startIndex));

    if (word[1] === TERM_WEIGHT) {
      teaser.push("</b>");
    }
  }
  teaser.push("…");
  return teaser.join("");
}

function formatSearchResultItem(item, terms) {
  let li = document.createElement("li");
  li.classList.add("search-results__item");
  li.innerHTML = `<a href="${item.ref}">${item.doc.title}</a>`;
  li.innerHTML += `<div class="search-results__teaser">${makeTeaser(
      item.doc.body, terms)}</div>`;
  return li;
}

// Go from the book view to the search view
function toggleSearchMode() {
  let $bookContent = document.querySelector(".book-content");
  let $searchContainer = document.querySelector(".search-container");
  if ($searchContainer.classList.contains("search-container--is-visible")) {
    $searchContainer.classList.remove("search-container--is-visible");
    document.body.classList.remove("search-mode");
    $bookContent.style.display = "block";
  } else {
    $searchContainer.classList.add("search-container--is-visible");
    document.body.classList.add("search-mode");
    $bookContent.style.display = "none";
    document.getElementById("search").focus();
  }
}

function initSearch() {
  let $searchInput = document.getElementById("search");
  if (!$searchInput) {
    return;
  }
  let $searchIcon = document.querySelector(".search-icon");
  $searchIcon.addEventListener("click", toggleSearchMode);

  let $searchResults = document.querySelector(".search-results");
  let $searchResultsHeader = document.querySelector(".search-results__header");
  let $searchResultsItems = document.querySelector(".search-results__items");
  const MAX_ITEMS = 10;

  let options = {
    bool: "AND",
    fields: {
      title: {boost: 2},
      body: {boost: 1},
    }
  };
  let currentTerm = "";
  let index = elasticlunr.Index.load(window.searchIndex);

  $searchInput.addEventListener("keyup", debounce(() => {
    let term = $searchInput.value.trim();
    if (term === currentTerm || !index) {
      return;
    }
    $searchResults.style.display = term === "" ? "none" : "block";
    $searchResultsItems.innerHTML = "";
    if (term === "") {
      return;
    }

    let results = index.search(term, options).filter((r) => r.doc.body !== "");
    if (results.length === 0) {
      $searchResultsHeader.innerText = `No search results for '${term}'.`;
      return;
    }

    currentTerm = term;
    $searchResultsHeader.innerText = `${results.length} search results for '${term}':`;
    for (let i = 0; i < Math.min(results.length, MAX_ITEMS); i++) {
      if (!results[i].doc.body) {
        continue;
      }

      $searchResultsItems.appendChild(
          formatSearchResultItem(results[i], term.split(" ")));
    }
  }, 150));
}

if (document.readyState === "complete" ||
    (document.readyState !== "loading" && !document.documentElement.doScroll)
) {
  initToggleMenu();
} else {
  document.addEventListener("DOMContentLoaded", () => {
    initToggleMenu();
    initSearch();
  });
}
