import com.codingfeline.buildkonfig.compiler.FieldSpec.Type.STRING
plugins {
    kotlin("multiplatform")
    id("com.android.library")
    kotlin("plugin.serialization") version "1.8.21"
    id("com.codingfeline.buildkonfig")
}

@OptIn(org.jetbrains.kotlin.gradle.ExperimentalKotlinGradlePluginApi::class)
kotlin {
    targetHierarchy.default()

    android {
        compilations.all {
            kotlinOptions {
                jvmTarget = "1.8"
            }
        }
    }
    
    listOf(
        iosX64(),
        iosArm64(),
        iosSimulatorArm64()
    ).forEach {
        it.binaries.framework {
            baseName = "shared"
        }
    }

    sourceSets {
        val commonMain by getting {
            dependencies {
                implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.5.0")
                implementation("io.ktor:ktor-client-core:2.2.4")
                implementation("io.ktor:ktor-client-content-negotiation:2.2.4")
                implementation("io.ktor:ktor-client-serialization:2.2.4")
                implementation("io.ktor:ktor-client-auth:2.2.4")
                implementation("io.ktor:ktor-client-logging:2.2.4")
                implementation("io.ktor:ktor-serialization-kotlinx-json:2.2.4")
                implementation("io.github.aakira:napier:2.6.1")
                implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.1")
                implementation("org.jetbrains.kotlinx:kotlinx-datetime:0.4.0")
                implementation("com.liftric:kvault:1.9.0")
            }
        }
        val commonTest by getting {
            dependencies {
                implementation(kotlin("test"))
            }
        }
        val androidMain by getting {
            dependencies {
                implementation("io.ktor:ktor-client-okhttp:2.2.4")
            }
        }
        val iosX64Main by getting
        val iosArm64Main by getting
        val iosSimulatorArm64Main by getting
        val iosMain by getting {
            dependsOn(commonMain)
            iosX64Main.dependsOn(this)
            iosArm64Main.dependsOn(this)
            iosSimulatorArm64Main.dependsOn(this)
            dependencies {
                implementation("io.ktor:ktor-client-darwin:2.2.4")
            }
        }
    }
}

buildkonfig {
    packageName = "se.lucasboberg.golf"
    exposeObjectWithName = "BuildKonfig"

    defaultConfigs {
        buildConfigField(STRING, "BASE_URL", "https://golf.up.railway.app")
        buildConfigField(STRING, "ENVIRONMENT", "dev")
    }

    defaultConfigs("dev") {
        buildConfigField(STRING, "BASE_URL", "https://golf.up.railway.app")
        buildConfigField(STRING, "ENVIRONMENT", "dev")
    }

    defaultConfigs("prod") {
        buildConfigField(STRING, "BASE_URL", "https://golf.up.railway.app")
        buildConfigField(STRING, "ENVIRONMENT", "prod")
    }
}

android {
    namespace = "se.lucasboberg.golf"
    compileSdk = 33
    defaultConfig {
        minSdk = 24
    }
}