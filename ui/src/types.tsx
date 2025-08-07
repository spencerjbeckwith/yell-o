export type VoiceCategories = "premade" | "generated" | "cloned" | "professional";

export interface Voice {
    category: VoiceCategories;
    description?: string | null;
    labels?: {
        accent?: string;
        age?: string;
        description?: string;
        gender?: string;
    };
    name: string;
    voice_id: string;
}