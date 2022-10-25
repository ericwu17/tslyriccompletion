import { useParams } from "react-router-dom";

export default function SongPage() {
  let { album, name } = useParams();

  return (
    <>
      <div>
        This is the song page!
      </div>
      <div>
        <h2>{album} -- {name}</h2>
      </div>
    </>
  );
}
