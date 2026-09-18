function copyToClipboard() {
  let copyText = document.getElementById("message")

  navigator.clipboard.write(copyText.innerText);

  console.log(copyText.innerText);
}
