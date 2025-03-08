#include <stdio.h>
#include <stdlib.h>
#include <dirent.h>
#include <string.h>

#define CACHE_FOLDER "./cache/"  // Pasta onde os arquivos tempor�rios (PMC) ser�o armazenados
#define MUSIC_FOLDER "./musicas/"  // Pasta onde as m�sicas MP3 est�o armazenadas

// Fun��o para listar os arquivos MP3 na pasta musicas/
void listar_musicas() {
    DIR *dir;
    struct dirent *entry;

    dir = opendir(MUSIC_FOLDER);
    if (dir == NULL) {
        printf("Erro ao abrir a pasta de m�sicas!\n");
        return;
    }

    printf("M�sicas dispon�veis:\n");
    while ((entry = readdir(dir)) != NULL) {
        if (strstr(entry->d_name, ".mp3") != NULL) {  // Filtra apenas arquivos MP3
            printf(" - %s\n", entry->d_name);
        }
    }

    closedir(dir);
}

// Fun��o de convers�o de MP3 para PCM (exemplo simples, voc� deve substituir com a implementa��o real)
int converter_mp3_para_pcm(const char *musica_escolhida) {
    // L�gica de convers�o de MP3 para PCM aqui (utilizando a biblioteca dr_mp3, por exemplo)
    // Supondo que a fun��o j� esteja implementada, o retorno seria 0 se a convers�o for bem-sucedida, e outro valor em caso de erro.

    // Apenas como exemplo, aqui voc� deve chamar a fun��o de convers�o real.
    printf("Convertendo MP3 para PCM: %s\n", musica_escolhida);

    // Simula��o de sucesso
    return 0;
}

// Fun��o para escolher a m�sica a partir da lista
void escolher_musica(char *musica_escolhida, size_t tamanho) {
    listar_musicas();  // Exibe as m�sicas dispon�veis

    // O usu�rio digita o nome da m�sica
    printf("\nDigite o nome da m�sica (com extens�o .mp3): ");
    scanf("%s", musica_escolhida);  // L� o nome da m�sica escolhida
}

int main() {
    char musica_escolhida[256];

    printf("Bem-vindo ao decodificador de MP3!\n");

    // Escolher a m�sica
    escolher_musica(musica_escolhida, sizeof(musica_escolhida));

    // Cria o caminho completo para o arquivo PCM dentro da pasta de cache
    char temp_filename[300];
    snprintf(temp_filename, sizeof(temp_filename), "%s%s.pcm", CACHE_FOLDER, musica_escolhida);  // Exemplo de extens�o .pcm

    // Aqui voc� pode usar a fun��o para converter o MP3 para PCM
    if (converter_mp3_para_pcm(musica_escolhida) == 0) {
        printf("Decodifica��o conclu�da com sucesso!\n");

        // Simulando o uso do arquivo tempor�rio (salvando o PCM tempor�rio)
        FILE *temp_file = fopen(temp_filename, "w");
        if (temp_file) {
            // Simula o processo de escrever o PCM no arquivo tempor�rio
            fprintf(temp_file, "Arquivo PCM tempor�rio para: %s\n", musica_escolhida);
            fclose(temp_file);

            printf("Arquivo PCM armazenado na pasta cache: %s\n", temp_filename);
        } else {
            printf("Erro ao criar o arquivo PCM tempor�rio.\n");
        }
    } else {
        printf("Ocorreu um erro na decodifica��o.\n");
    }

    return 0;
}
