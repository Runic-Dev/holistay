export interface BackendHandler {
  foo(): void
}

export class WebBackendHandler implements BackendHandler {
  foo(): void {
    console.log("WebBackendHandler says hi");
  }
}

export class TauriBackendHandler implements BackendHandler {
  foo(): void {
    console.log("TauriBackendHandler says hi");
  }
}
