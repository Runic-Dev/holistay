import type { Property } from "@/models/Property";

export class PropertyDataEvent {
  success: boolean;
  property: Property | null;
}

