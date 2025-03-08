#ifndef MP3_DECODER_H
#define MP3_DECODER_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>

#define DR_MP3_IMPLEMENTATION
#include "dr_mp3.h"

#define MUSIC_FOLDER "./musicas/"  // Pasta onde os arquivos MP3 estão armazenados
#define OUTPUT_PCM "./output.pcm"  // Arquivo PCM gerado

// Função para listar os arquivos MP3 na pasta
void listar_musicas() {
    DIR *dir;
    struct dirent *entry;

    dir = opendir(MUSIC_FOLDER);
    if (dir == NULL) {
        printf("Erro ao abrir a pasta de músicas!\n");
        return;
    }

    printf("Músicas disponíveis:\n");
    while ((entry = readdir(dir)) != NULL) {
        if (strstr(entry->d_name, ".mp3") != NULL) {  // Filtra apenas arquivos MP3
            printf(" - %s\n", entry->d_name);
        }
    }

    closedir(dir);
}

// Função para o usuário escolher uma música
void escolher_musica(char *musica_escolhida, size_t tamanho) {
    listar_musicas();
    printf("\nDigite o nome da música (com extensão .mp3): ");
    scanf("%255s", musica_escolhida);
}

// Função para converter MP3 em PCM e salvar no disco
int converter_mp3_para_pcm(const char *arquivo_mp3) {
    drmp3 decoder;
    char caminho_mp3[256];

    snprintf(caminho_mp3, sizeof(caminho_mp3), "%s%s", MUSIC_FOLDER, arquivo_mp3);

    // Inicializa o decodificador
    if (!drmp3_init_file(&decoder, caminho_mp3, NULL)) {
        printf("Erro ao abrir o arquivo MP3: %s\n", caminho_mp3);
        return -1;
    }

    // Abre o arquivo PCM para escrita
    FILE *pcmFile = fopen(OUTPUT_PCM, "wb");
    if (pcmFile == NULL) {
        printf("Erro ao criar o arquivo PCM!\n");
        drmp3_uninit(&decoder);
        return -1;
    }

    // Buffer para armazenar dados decodificados
    float buffer[4096];  // Tamanho do buffer de áudio
    size_t framesRead;

    // Loop para decodificar e salvar no arquivo PCM
    while ((framesRead = drmp3_read_pcm_frames_f32(&decoder, 4096, buffer)) > 0) {
        fwrite(buffer, sizeof(float), framesRead , pcmFile);
    }

    printf("Arquivo PCM gerado: %s\n", OUTPUT_PCM);

    // Fechar arquivos e liberar memória
    fclose(pcmFile);
    drmp3_uninit(&decoder);

    return 0;
}

#endif
