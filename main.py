from glob import glob
import os
import subprocess
from langcodes import *

# Définissez le chemin vers le dossier contenant les fichiers .mkv ici
path = "I:\Anime"

# Définissez le chemin vers le dossier où enregistrer les sous-titres ici
output_path = "E:\subtitles\\"

def fast_scandir(dirname):
    subfolders=[f.path for f in os.scandir(dirname) if f.is_dir()]
    for dirname in list(subfolders):
        subfolders.extend(fast_scandir(dirname))
    return subfolders
    

list_dir_path = fast_scandir(path)
list_dir_out = fast_scandir(output_path)
for i in range(len(list_dir_path)):
    list_dir_path[i] = list_dir_path[i].replace(path + "\\", "")
for i in range(len(list_dir_out)):
    list_dir_out[i] = list_dir_out[i].replace(output_path + "Arabic\\", "")
    list_dir_out[i] = list_dir_out[i].replace(output_path + "English\\", "")
    list_dir_out[i] = list_dir_out[i].replace(output_path + "French\\", "")
    list_dir_out[i] = list_dir_out[i].replace(output_path + "German\\", "")
    list_dir_out[i] = list_dir_out[i].replace(output_path + "Italian\\", "")
    list_dir_out[i] = list_dir_out[i].replace(output_path + "Portuguese\\", "")
    list_dir_out[i] = list_dir_out[i].replace(output_path + "Russian\\", "")
    list_dir_out[i] = list_dir_out[i].replace(output_path + "Spanish\\", "")

# list_dir_out = [item for item in list_dir_out if not 'Arabic' in item]
# list_dir_out = [item for item in list_dir_out if not 'English' in item]
# list_dir_out = [item for item in list_dir_out if not 'French' in item]
# list_dir_out = [item for item in list_dir_out if not 'German' in item]
# list_dir_out = [item for item in list_dir_out if not 'Italian' in item]
# list_dir_out = [item for item in list_dir_out if not 'Portuguese' in item]
# list_dir_out = [item for item in list_dir_out if not 'Russian' in item]
# list_dir_out = [item for item in list_dir_out if not 'Spanish' in item]

list_dir_path.sort()
# Parcourez chaque élément de la première liste
for element in list_dir_out:
    # Si l'élément est également présent dans la deuxième liste, supprimez-le de cette dernière
    if element in list_dir_path:
        list_dir_path.remove(element)
# Je coupe la liste de façons à ne garder que les X premiers éléments
list_dir_path = list_dir_path[:20]
# Parcourez tous les fichiers dans le dossier
for subdir in list_dir_path:
    for file in os.listdir(path + "\\" + subdir):
        # Vérifiez que le fichier est un fichier .mkv
        if file.endswith(".mkv"):
            path_to_file = path + "\\" + subdir + "\\"
            # Exécutez la commande ffmpeg pour récupérer les informations sur les sous-titres du fichier .mkv
            # command = f"ffmpeg -i \"{os.path.join(path, file)}\" -map 0:s 2>&1"
            command = f"ffprobe -loglevel error -select_streams s -show_entries stream=index:stream_tags=language -of csv=p=0 \"{os.path.join(path_to_file, file)}\""
            output = subprocess.run(command, shell=True,
                                    capture_output=True).stdout.decode()
            # Séparez la sortie en lignes
            lines = output.split("\n")

            # Créez un dictionnaire pour stocker les informations sur les sous-titres
            subtitles = {}
            # Parcourez toutes les lignes de la sortie
            for line in lines:
                # Séparez la ligne en deux parties : l'ID du sous-titre et la clé (la langue du sous-titre)
                parts = line.split(",")
                # Si la ligne contient une clé et un ID
                if len(parts) == 2:
                    # Ajoutez l'ID et la clé au dictionnaire
                    subtitles[parts[0]] = parts[1].rstrip("\r")

            # Suppression des valeurs en double
            temp=[]
            res={}
            for key,val in subtitles.items():
                if val not in temp:
                    temp.append(val)
                    res[key]=val
            subtitles = res

            # Parcourez tous les sous-titres du fichier .mkv
            for i in subtitles:
                lang = subtitles[i]
                name=file.rstrip(".mkv")
                name=name.replace("&nbsp;", "")
                name=name.replace("&amp", "")
                nn= file.split(" É")
                lang_alpha2 = Language.get(subtitles[i])
                lang_full = Language.get(lang_alpha2, normalize=False).display_name()
                dir = output_path + lang_full
                if not os.path.exists(dir + "\\" + subdir):
                    os.makedirs(dir + "\\" + subdir)
                out = dir + "\\" + subdir + "\\"
                # Exécutez la commande ffmpeg pour extraire le i-ème sous-titre du fichier .mkv
                command = f"ffmpeg -i \"{os.path.join(path_to_file, file)}\" -map 0:{i} -c:s copy \"{out + name + '_' + lang + '.ass'}\""
                subprocess.run(command)