plugins {
    application
}

repositories {
    mavenCentral()
}

dependencies {
	{{DEPENDENCIES}}
}

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(21)
    }
}

application {
    mainClass = "{{MAIN_CLASS}}"
}

tasks.withType<Test> {
    useJUnitPlatform()
}

