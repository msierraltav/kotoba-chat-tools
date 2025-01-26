namespace Kotoba.TauriPlugIn;

using static System.Environment;


public class Snippet{
    public string? Id { get; set; }
    public string? Content { get; set; }
}

public class SearchRequest{
    public string? Query { get; set; }
}

public class SearchResult{
    public string? Match {get; set;}
    public Snippet? Snippet {get; set;}
}

public class SnippetsController{

    private static readonly string Store = Path.Combine(GetFolderPath(SpecialFolder.Personal), "Kotoba");

    public SnippetsController(){
        if (!Directory.Exists(Store)){
            Directory.CreateDirectory(Store);
        }
    }

    public void Save(string snippet){
        var path = Path.Combine(Store, $"{Guid.NewGuid()}.txt");
        File.WriteAllText(path, snippet);
    }
}