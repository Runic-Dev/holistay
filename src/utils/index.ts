export function addBase64HtmlSyntax(encodedfile: string, mimeType: string): string {
  return `data:image/${mimeType};base64,${encodedfile}`;
}
