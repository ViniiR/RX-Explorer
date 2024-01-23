/// <reference types="vite/client" />

declare module "*.vue" {
    import type { DefineComponent } from "vue";
    const component: DefineComponent<{}, {}, any>;
    export default component;
}

type Disk = {
    availableCapacity: String;
    diskType: String;
    fileSystem: String;
    name: String;
    totalCapacity: String;
    usagePercentage: String;
};
