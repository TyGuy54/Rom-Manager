import "../App.css";


export const LandingPage = () => {
    return (
        <div className="info-grid-container">
            <h2 className="header-pos"></h2>
            <div className="content-pos">
                <div className="content-size">
                    <h1>Rom Manager Version - 1.0</h1>

                    <p>Recently Played games - </p>
                        {/* TODO - Add a way to to get the recantly played games, proaboly do it by adding a way to get the sot recacnly opened file */}
                    <p>Least Played games -</p>
                </div>
            </div>
        </div>
    );
}