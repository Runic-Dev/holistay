export function addBase64HtmlSyntax(encodedfile: string, mimeType: string): string {
  console.log("addBase64");
  return `data:image/${mimeType};base64,${encodedfile}`;
}
