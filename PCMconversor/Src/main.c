#include <stdio.h>
#include <stdlib.h>
#include <dirent.h>
#include <string.h>

#define CACHE_FOLDER "./cache/"  // Pasta onde os arquivos temporários (PMC) serão armazenados
#define MUSIC_FOLDER "./musicas/"  // Pasta onde as músicas MP3 estão armazenadas

// Função para listar os arquivos MP3 na pasta musicas/
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

// Função de conversão de MP3 para PCM (exemplo simples, você deve substituir com a implementação real)
int converter_mp3_para_pcm(const char *musica_escolhida) {
    // Lógica de conversão de MP3 para PCM aqui (utilizando a biblioteca dr_mp3, por exemplo)
    // Supondo que a função já esteja implementada, o retorno seria 0 se a conversão for bem-sucedida, e outro valor em caso de erro.

    // Apenas como exemplo, aqui você deve chamar a função de conversão real.
    printf("Convertendo MP3 para PCM: %s\n", musica_escolhida);

    // Simulação de sucesso
    return 0;
}

// Função para escolher a música a partir da lista
void escolher_musica(char *musica_escolhida, size_t tamanho) {
    listar_musicas();  // Exibe as músicas disponíveis

    // O usuário digita o nome da música
    printf("\nDigite o nome da música (com extensão .mp3): ");
    scanf("%s", musica_escolhida);  // Lê o nome da música escolhida
}

int main() {
    char musica_escolhida[256];

    printf("Bem-vindo ao decodificador de MP3!\n");

    // Escolher a música
    escolher_musica(musica_escolhida, sizeof(musica_escolhida));

    // Cria o caminho completo para o arquivo PCM dentro da pasta de cache
    char temp_filename[300];
    snprintf(temp_filename, sizeof(temp_filename), "%s%s.pcm", CACHE_FOLDER, musica_escolhida);  // Exemplo de extensão .pcm

    // Aqui você pode usar a função para converter o MP3 para PCM
    if (converter_mp3_para_pcm(musica_escolhida) == 0) {
        printf("Decodificação concluída com sucesso!\n");

        // Simulando o uso do arquivo temporário (salvando o PCM temporário)
        FILE *temp_file = fopen(temp_filename, "w");
        if (temp_file) {
            // Simula o processo de escrever o PCM no arquivo temporário
            fprintf(temp_file, "Arquivo PCM temporário para: %s\n", musica_escolhida);
            fclose(temp_file);

            printf("Arquivo PCM armazenado na pasta cache: %s\n", temp_filename);
        } else {
            printf("Erro ao criar o arquivo PCM temporário.\n");
        }
    } else {
        printf("Ocorreu um erro na decodificação.\n");
    }

    return 0;
}
